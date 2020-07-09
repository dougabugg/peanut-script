extern crate peanut_script_vm as vm;

pub mod stage0;

/*
STAGE 0
the stage0 AST doesn't do any type checking and is used to generate bytecode
for the VM to execute.

STAGE 1
the stage1 AST will have type checking and will generate a stage0 AST. the stage1
AST won't support user-defined types, but will check for any type errors that would
cause the program to crash when executing in the VM. this will also handle loop
labels, assigning indices to local variables, and handling constant expressions
and string literals.

STAGE 2
the stage2 AST will implement the user-facing type system and generate a stage1 AST.
the type system consists of: associated functions, run-time type information,
traits (interfaces), dynamic dispatch, and tagged unions.
*/

/*
ISSUES:
- if stage2 already does type checking, isn't it redundant for stage1 to also do
    type checking? should we skip implementing type checking in stage1?

currently, stage0 is very low-level, with the AST layout require special handling
(see notes in src/stage0/mod.rs).

I think we might want to re-write the stage0 AST, and implement some of the features
from stage1 into it, like variable indexing and loop labeling.
*/

/////////////////////////

/*
here's the new plan:
- re-write stage0 to support variable indexing and loop labeling, still no type
    checking.
- write stage1 to implement the user-facing type system, with type checking.
*/