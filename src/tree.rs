use scrivx_reader;
use scrivx_reader::Scrivening;

pub fn binder_tree() {
	let ref blueprint: Vec<Scrivening> = scrivx_reader::process_scrivx();
	println!("{}", "Binder");
	list_scriv_contents(&blueprint, 1, &mut vec![true]);
}

fn list_scriv_contents(blueprint: &Vec<Scrivening>, depth: i32, active_levels: &mut Vec<bool>) {
	for (index, scrivening) in blueprint.iter().enumerate() {
		let mut out: String = String::new();
		let mut i: i32 = 0;
		while i < depth {
			if i == depth - 1 {
				if index == blueprint.len() - 1 {
					out.push_str("└── ");
					active_levels[(depth - 1) as usize] = false;
				} else {
					out.push_str("├── ");
				}	
			} else {
				if active_levels[i as usize] {
					out.push_str("|   ");
				} else {
					out.push_str("    ");
				}	
			}		
			i += 1;
		}
		out.push_str(&format!("{} [{}]", &scrivening.get_title(), &scrivening.get_id()));
		println!("{}", out);
		match scrivening.get_children() {
			None => continue,
			Some(ref children) => {
				if depth >= active_levels.len() as i32{
					active_levels.push(true);
				} else {
					active_levels[depth as usize] = true;
				}
				list_scriv_contents(&children, depth + 1, active_levels);
			}
		}
	}
}