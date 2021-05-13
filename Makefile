
shader_compiler = glslangValidator

# All input shaders.
glsls = $(wildcard src/renderer/shaders/*.vert src/renderer/shaders/*.frag)

# All SPIR-V targets.
spirvs = $(addsuffix .spv,$(glsls))

.PHONY: default
default: $(spirvs)

# Rule for making a SPIR-V target.
$(spirvs): %.spv: %
	$(shader_compiler) -V $< -o $@

.PHONY: clean
clean:
	rm -f $(spirvs)
