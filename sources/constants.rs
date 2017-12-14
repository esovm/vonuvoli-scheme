

use super::values::exports::*;
use super::primitives_procedures::exports::*;

use std::f64;




pub mod exports {
	pub use super::*;
}




pub const NULL : ValueSingleton = ValueSingleton::Null;
pub const VOID : ValueSingleton = ValueSingleton::Void;
pub const UNDEFINED : ValueSingleton = ValueSingleton::Undefined;
pub const PORT_EOF : ValueSingleton = ValueSingleton::PortEof;

pub const TRUE : Boolean = Boolean (true);
pub const FALSE : Boolean = Boolean (false);


pub const NULL_VALUE : Value = Value::Singleton (VALUE_META_1, NULL, VALUE_META_2);
pub const VOID_VALUE : Value = Value::Singleton (VALUE_META_1, VOID, VALUE_META_2);
pub const UNDEFINED_VALUE : Value = Value::Singleton (VALUE_META_1, UNDEFINED, VALUE_META_2);
pub const PORT_EOF_VALUE : Value = Value::Singleton (VALUE_META_1, PORT_EOF, VALUE_META_2);

pub const TRUE_VALUE : Value = Value::Boolean (VALUE_META_1, TRUE, VALUE_META_2);
pub const FALSE_VALUE : Value = Value::Boolean (VALUE_META_1, FALSE, VALUE_META_2);




pub const ZERO : NumberInteger = NumberInteger (0);
pub const ONE : NumberInteger = NumberInteger (1);

pub const ZERO_REAL_POSITIVE : NumberReal = NumberReal (0.0);
pub const ZERO_REAL_NEGATIVE : NumberReal = NumberReal (-0.0);
pub const ONE_REAL_POSITIVE : NumberReal = NumberReal (1.0);
pub const ONE_REAL_NEGATIVE : NumberReal = NumberReal (-1.0);

pub const INF_POSITIVE : NumberReal = NumberReal (f64::INFINITY);
pub const INF_NEGATIVE : NumberReal = NumberReal (f64::NEG_INFINITY);
pub const NAN_POSITIVE : NumberReal = NumberReal (f64::NAN);
pub const NAN_NEGATIVE : NumberReal = NumberReal (f64::NAN);

pub const EPSILON_POSITIVE : NumberReal = NumberReal (0f64 + f64::EPSILON);
pub const EPSILON_NEGATIVE : NumberReal = NumberReal (0f64 - f64::EPSILON);




pub const CONSTANT_PROCEDURE_ATTRIBUTES_0 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (0),
				output : ProcedureOutputAttributes::Constant,
			};

pub const CONSTANT_PROCEDURE_ATTRIBUTES_1 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (1),
				output : ProcedureOutputAttributes::Constant,
			};

pub const CONSTANT_PROCEDURE_ATTRIBUTES_2 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (2),
				output : ProcedureOutputAttributes::Constant,
			};

pub const CONSTANT_PROCEDURE_ATTRIBUTES_3 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (3),
				output : ProcedureOutputAttributes::Constant,
			};

pub const CONSTANT_PROCEDURE_ATTRIBUTES_4 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (4),
				output : ProcedureOutputAttributes::Constant,
			};

pub const CONSTANT_PROCEDURE_ATTRIBUTES_5 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (5),
				output : ProcedureOutputAttributes::Constant,
			};

pub const CONSTANT_PROCEDURE_ATTRIBUTES_N : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Unbounded,
				output : ProcedureOutputAttributes::Constant,
			};

