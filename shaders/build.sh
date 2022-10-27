#!/bin/bash
glslangValidator basic_frag.glsl -V -S frag -o basic_frag.spv
glslangValidator basic_vert.glsl -V -S vert -o basic_vert.spv
glslangValidator basic_compute.glsl -V -S comp -o basic_compute.spv