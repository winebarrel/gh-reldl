PROGRAM := gh-reldl
SHELL   := /bin/bash
VERSION := v$(shell cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "$(PROGRAM)").version')
TARGET  := x86_64-apple-darwin
# TARGET  := aarch64-apple-darwin
# TARGET  := x86_64-unknown-linux-gnu
# TARGET  := aarch64-unknown-linux-gnu

target/$(TARGET)/release/$(PROGRAM):
	cross build --target $(TARGET) --release

.PHONY: package
package: target/$(TARGET)/release/$(PROGRAM)
	mv target/$(TARGET)/release/$(PROGRAM) $(PROGRAM)_$(VERSION)_$(TARGET)
	sha256sum $(PROGRAM)_$(VERSION)_$(TARGET) > $(PROGRAM)_$(VERSION)_$(TARGET).sha256sum

.PHONY: clean
clean:
	rm -rf gh-reldl* target
