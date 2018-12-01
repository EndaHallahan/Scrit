pub fn help(args: &[String]) {
	if args.is_empty() {

	} else {
		for query in args {
			match query.as_str() {
				"init" => {},
				"push" => {},
				"pull" => {},
				"tree" => {},
				"version" => {},
				"update" => {},
				"tutorial" => {},
				_ => {}
			}
		}
	}
}