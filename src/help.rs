pub fn help(args: &[String]) {
	if args.is_empty() {
		println!("
For more information on a command, type 'scrit help <command name>'. 
For a basic runthrough of how to use this program, type 'scrit help tutorial'.

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
Scrit can interact with a Scrivener project, and must be executed for each project you wish to use
with Scrit.
					")},
				"push" => {println!("
scrit push <documents> <options>

Compiles specified <documents> and uploads them to a Google Drive. Documents can be specified 
by name, or by id by prefacing the id with a #. To compile an entire project, use 'Binder'. 
Subdocuments of specified documents will be included in the compile unless otherwise specified. 

Options:
-omit (-o) 		Omit specified files from compilation. Argument is a comma-separated 
			list of file names or ids between quotation marks.
-include (-i)		Ignore files' include/exclude value from compile when compiling.
-split (-s)		Split pushed files into separate documents.
-clean (-c)		Pushes to GDocs without break placeholders. Documents exported 
			in this manner cannot be pulled back into Scrivener.
-directory (-d)		Specifies a filepath in the Google Drive to upload to. Defaults to the root.
					")},
				"pull" => {println!("
scrit pull <documents> <options>

Downloads specified documents from a Google Drive, decompiles them, and merges them with the local 
contents of a project. Coming soon!
					")},
				"tree" => {println!("
scrit tree

When in a .scriv folder, displays a filetree representation of the contents of a Scrivener project.
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
About this program:
-------------------
Scrit is an interface between Scrivener and Google Docs. It is meant to facilitate the transfer of items 
from one to the other for the purposes of editing or collaborative writing, and to streamline common 
workflows involving the two programs. This is done through two core mechanisms: Push and Pull. However,
before we can dive into these, we must first do a little bit of setup with scrit init.


Init
----
Before we can use Scrit on a project, we first need to initialize it. This is done through the init command:

	scrit init

Running this command sets up a few files that Scrit needs in order to function correctly. Note that it only 
has to be run once per project, and also that it must be run while inside the top level of that project's 
.scriv folder. In fact, all of the commands Scrit provides are meant to be run from this location.

For more information on the init command, type 'scrit help init'. To move on, type 'scrit help tutorial2'.

				")},
				"tutorial2" => {println!("
Push
----
Now that we have initialized Scrit, we can start using its other commands. We will start with Push, as that is 
likely the first command you will want to use.

The Push mechanism, put simply, allows you to take parts of any scrivener project and copy them to a 
Google Drive. To say it another way, you are taking your local documents and *push*ing them to a Google Drive,
from which they can be opened in Google Docs. It is quite simple to use, but offers a lot of versatility in 
what you wish to upload. Additionally, files will be placed inside a specially-created project folder, so
you don't have to worry about keeping them organized.

The Push mechanism is used through the Push command:

	scrit push <documents> <options>

Oh, a quick note about the syntax of this tutorial: parts of commands written in <angle brackets> are arguments,
meaning that you should replace them with something else. In this case, we have two sets of arguments, <documents> 
and <options>.

The <documents> argument is where you will specify what documents you wish to send to your Google Drive. Say, for 
instance, that I have in my project a file called 'example'. To push that to my Drive, I would type the following:

	scrit push example

You can also specify multiple documents, like so:

	scrit push example1 example2

By default, these documents will be compiled together into one document called example1, but that behaviour can
be changed with the -s option (more on options in a minute). Note also that each document specified this way will 
be given a title header in the compiled document.

Now, specifying documents by name can get tiresome. Worry not; you can also specify documents by their three-digit
ids. These can be found by running 'scrit tree' (a very handy command in general), and should be preceded by a 
pound sign:

	scrit push #123 #456 #789

Much more efficient.

One last thing: it is important to note that when you specify a document, all sub-documents under it in the tree
will be compiled into it as well. For example, if I have chapters as folders containing multiple scene documents, 
I can compile all of the scenes into one by specifying the chapter. Additionally, if you ever want to push your 
entire project to your Google Drive as a single document, this can be done by specifying 'Binder' in place of 
document names or ids.

The options argument comes after all of your documents. It is completely optional. These commands affect the way
the document is pushed. Option arguments are preceded by a -, and you can have as many as you wish. Additionally, 
they come in both long and short form. For example, the 'clean' option can be specified as either '-clean' or '-c':

	scrit push example1 example2 -c

To see a list of these options, and to see more about the push command itself, type 'scrit help push'. To move on,
type 'scrit help tutorial3'.

				")},
				"tutorial3" => {println!("
Pull
----
Pull is the reverse of Push: it takes documents on your Google Drive and imports (or pulls) them into your
Scrivener project. However, it is important to note that you can only pull documents that you (or someone else) has
pushed using Scrit. This is because pushed documents contain something very important: break placeholders.

If you have already pushed a document to your Google Drive, you may have noticed something odd in the text, like
this:

	[[[123]]]

This is a break placeholder. They are special markers that tell Scrit where each document or subdocument begins and 
ends. These are *very important* to the Pull mechanism, so please do not delete or alter them. Also, these cannot be
replicated through just text, so don't bother trying.

Now then, the pull command looks like this:

	scrit pull <files>

Simple, right? There's only one argument this time: <files>. As you might have guessed, this is where you specify
the names of the files you want to import back into Scrivener. They will be decompiled back into their component 
pieces, along with any changes you may have made, and merged with what's already in your project.

Note that I said merge, not replace. This is different to Push. Using the push command will replace files of the 
same name on your Google Drive; however, pulling documents from your Drive does not automatically replace files in 
your project. Instead, Scrit will compare the two documents and *merge* them. This means that you can still make 
changes to your project locally without having to worry about losing them to changes made on the Drive.

However, this poses one problem: what if the same piece of text has been changed on both the local copy and the 
copy on the Drive? Worry not; if this happens, you will be shown both versions and given the choice of which to 
accept.

And that's the basics of Scrit! For more information on the pull command, type 'scrit help push'. You've probably
noticed by now that you can recieve an explanation of any command by typing it after 'scrit help'. Additionally, 
just typing 'scrit help' by itself will give you a full list of all the commands scrit provides.

Good luck, and happy trails!
					")},
				_ => {}
			}
		}
	}
}