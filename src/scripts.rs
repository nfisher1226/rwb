pub struct Scripts {
    pub disable_forms: String,
    pub enable_forms: String,
}

impl Scripts {
    pub fn new() -> Scripts {
	let disable_forms = String::from("inputs = document.getElementsByTagName('input');
	    for (i = 0; i < inputs.length; i++) { inputs[i].disabled = true; }
	    edits = document.getElementsByTagName('textarea');
	    for (i = 0; i < edits.length; i++) { edits[i].disabled = true; }");
	let enable_forms = String::from("inputs = document.getElementsByTagName('input');
	    for (i = 0; i < inputs.length; i++) { inputs[i].disabled = false; }
	    edits = document.getElementsByTagName('textarea');
	    for (i = 0; i < edits.length; i++) { edits[i].disabled = false; }");
	Scripts {
	    disable_forms,
	    enable_forms,
	}
    }
}
