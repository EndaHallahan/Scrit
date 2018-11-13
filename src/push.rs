use drive_operations;

pub fn push(args: Vec<String>) {
	let hub = drive_operations::get_hub();
}

/*
scrit push <files> <options>

Options:
-omit (-o) 		Omit specified files from compilation. Argument is a comma-separated 
				 list of file names or ids.
-include (-i)	Ignore files' include/exclude value from compile when compiling.
-split (-s)		Split files into separate documents on break point.
-break (-b)		Breaks documents in specified point. In normal operation, this adds 
				 page breaks and titles. Behaviour is different if -s is 
				 signified. Argument is an integer. Default is no breaks.
-clean (-c)		Pushes to GDocs without break placeholders. Documents exported 
				 in this manner cannot be pulled back into Scrivener.
-directory (-d)	Specefies a filepath in the Google Drive to upload to. Defaults to the root.
*/

