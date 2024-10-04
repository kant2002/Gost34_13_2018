import Lake
open Lake DSL

package "GostLean" where
  -- add package configuration options here

lean_lib «GostLean» where
  -- add library configuration options here

@[default_target]
lean_exe "gostlean" where
  root := `Main
