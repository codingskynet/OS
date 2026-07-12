ROOTFS := rootfs
INITARFS := ../artifacts/initarfs

.PHONY: initarfs ports-prepare ports-sync ports-status ports-verify ports-clean FORCE

initarfs: $(INITARFS)

$(INITARFS): FORCE
	mkdir -p $(dir $@)
	COPYFILE_DISABLE=1 tar --format ustar -cf $@.tmp -C $(ROOTFS) .
	cmp -s $@.tmp $@ || mv $@.tmp $@
	rm -f $@.tmp

FORCE:

# Pass PORTS="micropython ..." to operate on selected ports only.
ports-prepare:
	python3 ../scripts/ports.py prepare $(PORTS)

ports-sync:
	python3 ../scripts/ports.py sync $(PORTS)

ports-status:
	python3 ../scripts/ports.py status $(PORTS)

ports-verify:
	python3 ../scripts/ports.py verify $(PORTS)

ports-clean:
	python3 ../scripts/ports.py clean $(PORTS)
