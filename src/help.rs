pub fn help(args: &[String]) {
	if args.is_empty() {
		println!("
For more information on a command, type 'scrit help <command name>'. 
For a quick runthrough of how to use this program, type 'scrit help tutorial'.

init 		Initializes Scrit in a .scriv directory.
push 		Pushes documents from a Scrivener project to a Google Drive.
pull 		Pulls documents from a Google Drive and imports them into a Scrivener project.
tree 		Displays a filetree representation of a Scrivener project.
info 		Displays information about Scrit, including the installed version number.
update 		Checks if an updated version of Scrit is available.
help 		Displays help information for Scrit commands.
			");
	} else {
		for query in args {
			match query.as_str() {
				"init" => {println!("
scrit init

When in a .scriv folder, initializes Scrit for that project. This command must be executed before 
Scrit can interact with a scrivener project, and must be executed for each project you wish to use
with Scrit.
					")},
				"push" => {println!("
scrit push <documents> <options>

Compiles specified <documents> and uploads them to a Google Drive. Documents can be specified 
by name, or by id by prefacing the id with a #. Subdocuments of specified documents will be included
in the compile unless otherwise specified.

Options:
-omit (-o) 		Omit specified files from compilation. Argument is a comma-separated 
				 list of file names or ids.
-include (-i)	Ignore files' include/exclude value from compile when compiling.
-split (-s)		Split pushed files into separate documents.
-clean (-c)		Pushes to GDocs without break placeholders. Documents exported 
				 in this manner cannot be pulled back into Scrivener.
-directory (-d)	Specifies a filepath in the Google Drive to upload to. Defaults to the root.
					")},
				"pull" => {println!("
scrit pull <documents> <options>

Downloads specified documents from a Google Drive, decompiles them, and merges them with the local 
contents of a project. Coming soon!
					")},
				"tree" => {println!("
scrit tree

When in a .scriv folder, displays a filetree representation of the contents of a scrivener project.
Files are displayed in the format 'filename [id]'.
					")},
				"info" => {println!("
scrit info

Displays information about Scrit, including the installed version number.
					")},
				"update" => {println!("
scrit update

Checks if an update to Scrit is available. If one is, you will be prompted to update.
					")},
				"tutorial" => {println!("
Coming soon!
					")},
				_ => {}
			}
		}
	}
}