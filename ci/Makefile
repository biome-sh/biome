# Keep plans in a correct build order
PLANS := components/bio components/launcher components/sup components/plan-build components/backline components/studio components/pkg-export-container components/pkg-export-tar

.DEFAULT_GOAL := all
all: tomlcheck shellcheck rendercheck build
.PHONY: all tomlcheck shellcheck rendercheck build
.PHONY: upload promote-testing promote-stable

setup: /.studio
setup: $(addprefix /bio/bin/,git tomlcheck shellcheck)
setup: $(addprefix /bio/bin/bio-plan-,tomlcheck shellcheck rendercheck)
.PHONY: setup

dist: /.studio
	@. results/bio.env && tar -zcf results/$$pkg_name-$$pkg_version-$$pkg_target.tar.gz -C /bio/pkgs/$$pkg_ident/bin bio
	@. results/bio.env && tar -zcf results/meta-$$pkg_version-$$pkg_target.tar.gz results/*.env
.PHONY: dist

# pre-commit:
#	@pre-commit run -a

tomlcheck: $(addprefix tomlcheck/,$(PLANS))
tomlcheck/%:
	@bio-plan-tomlcheck $*

shellcheck: $(addprefix shellcheck/,$(PLANS))
shellcheck/%:
	@bio-plan-shellcheck $*

rendercheck: $(addprefix rendercheck/,$(PLANS))
rendercheck/%:
	@bio-plan-rendercheck $*

build: $(addprefix build/,$(PLANS))
build/%: /.studio
	@bio-plan-build $*
	@mv results/last_build.env results/$$(basename $*).env
.NOTPARALLEL: $(addprefix build/,$(PLANS))

upload: $(addprefix upload/,$(PLANS))
upload/%:
	@. results/$$(basename $*).env && bio pkg upload results/$$pkg_artifact

promote-testing: $(addprefix promote-testing/,$(PLANS))
promote-testing/%:
	@. results/$$(basename $*).env && bio pkg promote $$pkg_ident testing

promote-stable: $(addprefix promote-stable/,$(PLANS))
promote-stable/%:
	@. results/$$(basename $*).env && bio pkg promote $$pkg_ident stable

# Executables
/bio/bin/git:
	@bio pkg install -fb core/git

# /bio/bin/pre-commit:
#	@bio pkg install -fb ya/pre-commit

/bio/bin/tomlcheck:
	@bio pkg install -fb ya/tomlcheck -c stable

/bio/bin/shellcheck:
	@bio pkg install -fb core/shellcheck

/bio/bin/bio-plan-%:
	@bio pkg install -fb biome/bio-sdk -c stable

# Studio guard: fail build if we are not in studio
/.studio:
	@echo "Looks like you are not in studio but studio is required."
	@exit 1

.PHONY: clean
clean:
	@rm -rf results/
