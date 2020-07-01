macro_rules! new_bin_op {
    ($name:ident) => {
        pub struct $name {
            pub lhs: u8,
            pub rhs: u8,
            pub out: u8,
        }

        impl $name {
            pub fn new(lhs: u8, rhs: u8, out: u8) -> $name {
                $name { lhs, rhs, out }
            }
        }

        impl DataIO for $name {
            type Target = (u8, u8, u8);
            fn from_bytes(t: Self::Target) -> Option<Self> {
                Some($name {
                    lhs: t.0,
                    rhs: t.1,
                    out: t.2,
                })
            }
            fn into_bytes(&self) -> Self::Target {
                (self.lhs, self.rhs, self.out)
            }
        }
    };
}

macro_rules! new_unary_op {
    ($name:ident) => {
        pub struct $name {
            pub val: u8,
            pub out: u8,
        }

        impl $name {
            pub fn new(val: u8, out: u8) -> $name {
                $name { val, out }
            }
        }

        impl DataIO for $name {
            type Target = (u8, u8);
            fn from_bytes(t: Self::Target) -> Option<Self> {
                Some($name { val: t.0, out: t.1 })
            }
            fn into_bytes(&self) -> Self::Target {
                (self.val, self.out)
            }
        }
    };
}

// TODO make a macro to create structs with arbitrary fields
