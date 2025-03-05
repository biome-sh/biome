use std::iter::FromIterator;

use rand::prelude::IndexedRandom;

use crate::btest;
use biome_butterfly::{member::Health,
                        rumor::{election::ElectionStatus,
                                ConstIdRumor as _,
                                Election}};
use biome_common::FeatureFlag;

#[test]
fn three_members_run_election() {
    let mut net = btest::SwimNet::new_rhw(3);
    net.mesh_mlw_smr();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");

    net.add_election(0, "witcher");
    net.add_election(1, "witcher");
    net.add_election(2, "witcher");

    assert_wait_for_election_status!(net, [0..3], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..3, 0..3], "witcher.prod");
}

#[test]
fn three_members_run_election_from_one_starting_rumor() {
    let mut net = btest::SwimNet::new_rhw(3);
    net.mesh_mlw_smr();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");
    net.add_election(0, "witcher");
    assert_wait_for_election_status!(net, [0..3], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..3, 0..3], "witcher.prod");
}

#[test]
fn five_members_elect_a_new_leader_when_the_old_one_dies() {
    let mut net = btest::SwimNet::new_rhw(5);
    net.mesh_mlw_smr();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");
    net.add_service(3, "core/witcher/1.2.3/20161208121212");
    net.add_service(4, "core/witcher/1.2.3/20161208121212");
    net.add_election(0, "witcher");
    assert_wait_for_election_status!(net, [0..5], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..5, 0..5], "witcher.prod");

    let leader_id = net[0].election_store
                          .lock_rsr()
                          .service_group("witcher.prod")
                          .map_rumor(Election::const_id(), |e| e.member_id.clone());

    let mut paused = 0;
    for (index, server) in net.iter_mut().enumerate() {
        if let Some(ref leader_id) = leader_id {
            if server.member_id() == leader_id {
                paused = index;
            }
        }
    }
    net[paused].pause();
    let paused_id = net[paused].member_id();
    assert_wait_for_health_of_mlr!(net, paused, Health::Confirmed);
    if paused == 0 {
        net[1].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    } else {
        net[0].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    }

    for i in 0..5 {
        if !i == paused {
            assert_wait_for_election_status!(net, i, "witcher.prod", ElectionStatus::Running);
        }
    }

    for i in 0..5 {
        if !i == paused {
            assert_wait_for_election_status!(net, i, "witcher.prod", ElectionStatus::Finished);
        }
    }

    net[if paused == 0 { 1 } else { 0 }].election_store
                                        .lock_rsr()
                                        .service_group("witcher.prod")
                                        .map_rumor(Election::const_id(), |e| {
                                            assert_eq!(e.term, 1);
                                            assert_ne!(e.member_id, paused_id);
                                        });
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn five_members_elect_a_new_leader_when_they_are_quorum_partitioned() {
    let mut net = btest::SwimNet::new_with_suitability_rhw(vec![1, 0, 0, 0, 0]);
    net[0].myself().lock_smw().set_persistent();
    net[4].myself().lock_smw().set_persistent();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");
    net.add_service(3, "core/witcher/1.2.3/20161208121212");
    net.add_service(4, "core/witcher/1.2.3/20161208121212");
    net.add_election(0, "witcher");
    net.connect_smr(0, 1);
    net.connect_smr(1, 2);
    net.connect_smr(2, 3);
    net.connect_smr(3, 4);
    assert_wait_for_health_of_mlr!(net, [0..5, 0..5], Health::Alive);
    assert_wait_for_election_status!(net, [0..5], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..5, 0..5], "witcher.prod");

    let leader_id = net[0].election_store
                          .lock_rsr()
                          .service_group("witcher.prod")
                          .map_rumor(Election::const_id(), |e| e.member_id.clone());

    assert_eq!(leader_id, Some(net[0].member_id().to_string()));

    let mut leader_index = 0;
    for (index, server) in net.iter_mut().enumerate() {
        if let Some(ref leader_id) = leader_id {
            if server.member_id() == leader_id {
                leader_index = index;
            }
        }
    }
    println!("Leader index: {}", leader_index);

    net.partition(0..2, 2..5);
    assert_wait_for_health_of_mlr!(net, [0..2, 2..5], Health::Confirmed);
    net[0].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    net[2].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    net[3].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    net[4].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    assert_wait_for_election_status!(net, 0, "witcher.prod", ElectionStatus::NoQuorum);
    assert_wait_for_election_status!(net, 1, "witcher.prod", ElectionStatus::NoQuorum);
    assert_wait_for_election_status!(net, 2, "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_election_status!(net, 3, "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_election_status!(net, 4, "witcher.prod", ElectionStatus::Finished);
    net[0].election_store
          .lock_rsr()
          .service_group("witcher.prod")
          .map_rumor(Election::const_id(), |e| {
              println!("OLD: {:#?}", e);
          });
    let new_leader_id = net[2].election_store
                              .lock_rsr()
                              .service_group("witcher.prod")
                              .map_rumor(Election::const_id(), |e| {
                                  println!("NEW: {:#?}", e);
                                  e.member_id.clone()
                              });
    assert!(leader_id.is_some());
    assert!(leader_id != new_leader_id);
    println!("Leader {:?} New {:?}", leader_id, new_leader_id);
    net.unpartition(0..2, 2..5);
    assert_wait_for_health_of_mlr!(net, [0..5, 0..5], Health::Alive);
    assert_wait_for_election_status!(net, 0, "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_election_status!(net, 1, "witcher.prod", ElectionStatus::Finished);

    net[4].election_store
          .lock_rsr()
          .service_group("witcher.prod")
          .map_rumor(Election::const_id(), |e| println!("MAJORITY: {:#?}", e));

    net[0].election_store
          .lock_rsr()
          .service_group("witcher.prod")
          .map_rumor(Election::const_id(), |e| {
              println!("MINORITY: {:#?}", e);
              assert_eq!(new_leader_id.as_ref(), Some(&e.member_id));
          });
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn three_persistent_members_reelect_same_leader_follower_partition() {
    // Server '0' will be the leader always.
    let mut net = btest::SwimNet::new_with_suitability_rhw(vec![1, 0, 0]);
    net[0].myself().lock_smw().set_persistent();
    net[1].myself().lock_smw().set_persistent();
    net[2].myself().lock_smw().set_persistent();

    net.add_service(0, "core/foobar/1.2.3/20241010101010");
    net.add_service(1, "core/foobar/1.2.3/20241010101010");
    net.add_service(2, "core/foobar/1.2.3/20241010101010");
    net.add_election(0, "foobar");
    net.connect_smr(0, 1);
    net.connect_smr(1, 2);
    assert_wait_for_health_of_mlr!(net, [0..3, 0..3], Health::Alive);
    assert_wait_for_election_status!(net, [0..3], "foobar.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..3, 0..3], "foobar.prod");

    let leader_id = net[0].election_store
                          .lock_rsr()
                          .service_group("foobar.prod")
                          .map_rumor(Election::const_id(), |e| e.member_id.clone());

    assert_eq!(leader_id, Some(net[0].member_id().to_string()));

    net.partition(0..2, 2..3);
    assert_wait_for_health_of_mlr!(net, [0..2, 2..3], Health::Confirmed);
    assert_wait_for_election_status!(net, 0, "foobar.prod", ElectionStatus::Finished);
    assert_wait_for_election_status!(net, 1, "foobar.prod", ElectionStatus::Finished);

    net[2].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    assert_wait_for_election_status!(net, 2, "foobar.prod", ElectionStatus::NoQuorum);

    net.unpartition(0..2, 2..3);
    assert_wait_for_health_of_mlr!(net, [0..2, 2..3], Health::Alive);
    assert_wait_for_election_status!(net, 2, "foobar.prod", ElectionStatus::Finished);

    let new_leader_id = net[2].election_store
                              .lock_rsr()
                              .service_group("foobar.prod")
                              .map_rumor(Election::const_id(), |e| e.member_id.clone());

    assert_eq!(leader_id, new_leader_id,
               "OLD: {:?}, NEW: {:?}",
               leader_id, new_leader_id);
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn five_persistent_members_same_leader_multiple_non_quorum_partitions() {
    let mut net = btest::SwimNet::new_with_suitability_rhw(vec![1, 0, 0, 0, 0]);
    for i in 0..5 {
        net[i].myself().lock_smw().set_persistent();
        net.add_service(i, "core/foobar/1.2.3/20241010101010");
    }
    net.add_election(0, "foobar");

    for servers in Vec::from_iter(0..5).windows(2) {
        net.connect_smr(servers[0], servers[1])
    }
    assert_wait_for_health_of_mlr!(net, [0..5, 0..5], Health::Alive);
    assert_wait_for_election_status!(net, [0..5], "foobar.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..3, 0..3], "foobar.prod");

    let leader_id = net[0].election_store
                          .lock_rsr()
                          .service_group("foobar.prod")
                          .map_rumor(Election::const_id(), |e| e.member_id.clone());

    assert_eq!(leader_id, Some(net[0].member_id().to_string()));

    // Making sure - running multiple times after a subset of follower (non-quorum) is partitioned
    // and reconnected the leader stays the same.
    let mut rng = rand::rng();
    let idxes = Vec::from_iter(1_usize..5_usize).choose_multiple(&mut rng, 2)
                                                .copied()
                                                .collect::<Vec<usize>>();
    for idx in idxes.iter() {
        println!("idx: {}", idx);
        net.partition_node(*idx);
        assert_wait_for_health_of_mlr!(net, *idx, Health::Confirmed);
    }
    let first = idxes[0];
    net[first].restart_elections_rsw_mlr_rhw_msr(FeatureFlag::empty());
    assert_wait_for_election_status!(net, 0, "foobar.prod", ElectionStatus::Finished);

    assert_wait_for_election_status!(net, first, "foobar.prod", ElectionStatus::NoQuorum);

    for idx in idxes.iter() {
        net.unpartition_node(*idx);
        assert_wait_for_health_of_mlr!(net, *idx, Health::Alive);
    }
    assert_wait_for_election_status!(net, first, "foobar.prod", ElectionStatus::Finished);

    let new_leader_id = net[first].election_store
                                  .lock_rsr()
                                  .service_group("foobar.prod")
                                  .map_rumor(Election::const_id(), |e| e.member_id.clone());

    assert_eq!(leader_id, new_leader_id,
               "OLD: {:?}, NEW: {:?}",
               leader_id, new_leader_id);
}
