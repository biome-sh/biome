installed_version() {
  bio --version | cut -d'/' -f1
}

installed_target() {
  local origin="${1:-core}"

  version_release="$(bio --version | cut -d' ' -f2)"
  version="$(cut -d'/' -f1 <<< "$version_release")"
  release="$(cut -d'/' -f2 <<< "$version_release")"
  cat /bio/pkgs/"${origin}"/bio/"$version"/"$release"/TARGET
}

get_bio_sup_version() {
  local origin="${1:-core}"
  if bio pkg list "$origin"/bio-sup &>/dev/null; then
    bio pkg list "$origin"/bio-sup | head -1 | awk '{print $1"/"$2}'
  else
    echo "Not installed"
  fi
}

get_bio_launcher_version() {
  local origin="${1:-core}"
  if bio pkg list "$origin"/bio-launcher &>/dev/null; then
    bio pkg list "$origin"/bio-launcher | head -1 | awk '{print $1"/"$2}'
  else
    echo "Not installed"
  fi
}

create_systemd_service() {
  sudo tee -a /etc/systemd/system/bio-sup.service << 'EOF'
[Unit]
Description=Biome Supervisor

[Service]
ExecStart=/bin/bio sup run
ExecStop=/bin/bio sup term
KillMode=process

[Install]
WantedBy=default.target
EOF
  sudo systemctl daemon-reload
  sudo systemctl unmask bio-sup
}

# Setup function runs before each test
setup_file() {
  echo "starting setup"
  sudo rm -f /bin/bio
  sudo rm -f /usr/bin/bio
  sudo rm -rf /bio/pkgs/core/bio
  sudo rm -rf /bio/pkgs/biome/bio
  sudo rm -rf /bio/pkgs/core/bio-sup
  sudo rm -rf /bio/pkgs/biome/bio-sup
  sudo rm -rf /bio/pkgs/core/bio-launcher
  sudo rm -rf /bio/pkgs/biome/bio-launcher
}

# Teardown function runs after each test
teardown_file() {
  # Stop any running Biome services
  if systemctl is-active bio-sup &>/dev/null; then
    echo "Stopping bio-sup service"
    sudo systemctl stop bio-sup
  fi
  
  # Remove systemd service file if it exists
  if [ -f /etc/systemd/system/bio-sup.service ]; then
    echo "Removing bio-sup service file"
    sudo rm -f /etc/systemd/system/bio-sup.service
    sudo systemctl daemon-reload
  fi
  
  echo "Teardown complete"
}

@test "Install core packages and prepare for migration" {
  # First install core packages
  run sudo components/bio/install.sh -v 1.6.1245
  [ "$status" -eq 0 ]
  [ "$(installed_target)" == "x86_64-linux" ]
  
  # Install core/bio-sup
  run sudo -E bio pkg install core/bio-sup --channel stable
  [ "$status" -eq 0 ]
  [ "$(get_bio_sup_version)" != "Not installed" ]
  
  # Install core/bio-launcher
  run sudo -E bio pkg install core/bio-launcher --channel stable
  [ "$status" -eq 0 ]
  [ "$(get_bio_launcher_version)" != "Not installed" ]
  
  # Create and start systemd service
  sudo bio license accept
  create_systemd_service
  run sudo systemctl start bio-sup
  [ "$status" -eq 0 ]
  sleep 5  # Give time for the service to start
  
  run sudo systemctl is-active bio-sup
  [ "$status" -eq 0 ]
}

@test "Migrate from core to biome packages" {
  # Store initial versions
  initial_bio_version="$(installed_version)"
  initial_bio_sup_version="$(get_bio_sup_version)"
  initial_bio_launcher_version="$(get_bio_launcher_version)"
  
  echo "Initial bio version: $initial_bio_version"
  echo "Initial bio-sup version: $initial_bio_sup_version"
  echo "Initial bio-launcher version: $initial_bio_launcher_version"
  
  run sudo -E components/bio/migrate.sh
  echo "Output from migration run:"
  echo "$output"
  [ "$status" -eq 0 ]
  
  # Check that biome packages are now installed
  [ "$(installed_target biome)" == "x86_64-linux" ]
  [ "$(get_bio_sup_version biome)" != "Not installed" ]
  [ "$(get_bio_launcher_version biome)" != "Not installed" ]
  
  # Verify systemd service is still running
  run systemctl is-active bio-sup
  [ "$status" -eq 0 ]
  
  # Verify that the running bio-sup process is from the biome origin
  run pgrep -a bio-sup
  [ "$status" -eq 0 ]
  [[ "$output" =~ /bio/pkgs/biome/bio-sup ]]
  
  # Check that we're now using biome packages, not core
  new_bio_version="$(installed_version)"
  [[ "$new_bio_version" =~ ^bio\ 2\.[0-9]+\.[0-9]+$ ]]
}

@test "Running migrate.sh again should not restart the bio-sup process" {
  # Get the current PID of bio-sup before running migrate.sh again
  run pgrep -x bio-sup
  [ "$status" -eq 0 ]
  bio_sup_pid_before=$output
  echo "bio-sup PID before second migration: $bio_sup_pid_before"
  
  # Run migrate.sh again
  run sudo -E components/bio/migrate.sh
  echo "Output from second migration run:"
  echo "$output"
  [ "$status" -eq 0 ]
  
  # Check that bio-sup is still running
  run systemctl is-active bio-sup
  [ "$status" -eq 0 ]
  
  # Get the PID of bio-sup after running migrate.sh again
  run pgrep -x bio-sup
  [ "$status" -eq 0 ]
  bio_sup_pid_after=$output
  echo "bio-sup PID after second migration: $bio_sup_pid_after"
  
  # Verify that the PID hasn't changed
  [ "$bio_sup_pid_before" = "$bio_sup_pid_after" ]
}
