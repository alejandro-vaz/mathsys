//^
//^ PRELUDE
//^

//> PRELUDE -> CRATE
pub use crate::{
    Settings,
    context::{
        infinite::Infinite,
        integer::Integer,
        natural::Natural,
        nexists::Nexists,
        rational::Rational,
        tensor::Tensor,
        undefined::Undefined,
        variable::Variable,
        whole::Whole
    },
    data::{
        _absolute::_Absolute,
        _annotation::_Annotation,
        _casts::_Casts,
        _declaration::_Declaration,
        _definition::_Definition,
        _equation::_Equation,
        _expression::_Expression,
        _factor::_Factor,
        _infinite::_Infinite,
        _limit::_Limit,
        _nest::_Nest,
        _node::_Node,
        _rational::_Rational,
        _start::_Start,
        _tensor::_Tensor,
        _term::_Term,
        _undefined::_Undefined,
        _use::_Use,
        _variable::_Variable,
        _whole::_Whole
    },
    lib::{
        class::Class,
        group::Group,
        object::Object,
        opcode::Opcode,
        pointer::Pointer,
        reparser::Reparser,
        runtime::Runtime,
        sign::Sign,
        stack::{
            exit,
            write
        },
        stdout::{
            stdout,
            Code
        },
        tip::Tip
    }
};

//> PRELUDE -> NUM_BIGINT
pub use num_bigint::BigUint;

//> PRELUDE -> NUM_INTEGER
pub use num_integer::Integer as Number;

//> PRELUDE -> NUM_TRAITS
pub use num_traits::{
    Zero,
    One
};

//> PRELUDE -> STD
pub use std::{
    fmt,
    ops,
    collections::{
        HashMap,
        HashSet
    },
    time::Instant,
    any::type_name
};

//> PRELUDE -> MINIZ_OXIDE
pub use miniz_oxide::inflate::{
    decompress_to_vec as decompress
};

//> PRELUDE -> BITVEC
pub use bitvec::{
    prelude::{
        BitVec,
        Lsb0,
        BitSlice
    },
    field::BitField
};

//> PRELUDE -> STRUM_MACROS
pub use strum_macros::AsRefStr;