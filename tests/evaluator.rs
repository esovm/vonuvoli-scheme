

extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	let expressions = vec! [
			
			Expression::Void,
			Expression::Value (NULL),
			
			Expression::ContextDefine ("a".into (), ZERO.into ()),
			Expression::ContextSelect ("a".into ()),
			Expression::ContextUpdate ("a".into (), ONE.into ()),
			Expression::ContextSelect ("a".into ()),
			
			(BooleanPrimitive1::Not, TRUE) .into (),
			
			(BooleanPrimitiveN::And, (TRUE, TRUE)) .into (),
			
		];
	
	let mut context = Context::new (None);
	let mut evaluator = Evaluator::new ();
	
	for expression in expressions {
		println! (">> {:?}", expression);
		let outcome = evaluator.evaluate (&mut context, &expression);
		match outcome {
			Ok (value) =>
				println! ("== {:?} {}", expression, value),
			Err (error) =>
				println! ("== {:?} {}", expression, error),
		}
	}
	
}
