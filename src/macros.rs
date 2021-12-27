#[macro_export]
macro_rules! sexpr_test_case {
	(name: $name:ident,input: $input:expr,value: $value:expr) => {
		paste::paste! {
			#[test]
			fn [<test_deserialize_ $name>]() {
				fn assert_eq_parsed<'de, T>(input: &'de str, expected: &T)
				where
					T: std::fmt::Debug + serde::Deserialize<'de> + PartialEq
				{
					let parsed: T = serde_sexpr::from_str(input)
						.expect("Failed to parse input");
					pretty_assertions::assert_eq!(&parsed, expected);
				}

				let value = $value;
				assert_eq_parsed($input, &value);
			}

			#[test]
			fn [<test_serialize_ $name>]() {
				fn assert_eq_ugly<T>(input: &T, expected: &str)
				where
					T: ?Sized + serde::Serialize
				{
					let written = serde_sexpr::to_string(input)
						.expect("Failed to write input");
					pretty_assertions::assert_eq!(written.as_str(), expected);
				}

				let value = $value;
				assert_eq_ugly(&value, $input);
			}
		}
	};
}
