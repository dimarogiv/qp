
use builtin;
use str;

set edit:completion:arg-completer[querypath] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'querypath'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'querypath'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand add 'Add a new directory or increment its rank'
            cand edit 'Edit the database'
            cand import 'Import entries from another application'
            cand init 'Generate shell configuration'
            cand query 'Search for a directory in the database'
            cand remove 'Remove a directory from the database'
        }
        &'querypath;add'= {
            cand -s 'The rank to increment the entry if it exists or initialize it with if it doesn''t'
            cand --score 'The rank to increment the entry if it exists or initialize it with if it doesn''t'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;edit'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand decrement 'decrement'
            cand delete 'delete'
            cand increment 'increment'
            cand reload 'reload'
        }
        &'querypath;edit;decrement'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;edit;delete'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;edit;increment'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;edit;reload'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;import'= {
            cand --from 'Application to import from'
            cand --merge 'Merge into existing database'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;init'= {
            cand --cmd 'Changes the prefix of the `z` and `zi` commands'
            cand --hook 'Changes how often querypath increments a directory''s score'
            cand --no-cmd 'Prevents querypath from defining the `z` and `zi` commands'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;query'= {
            cand --exclude 'Exclude the current directory'
            cand -a 'Show unavailable directories'
            cand --all 'Show unavailable directories'
            cand -i 'Use interactive selection'
            cand --interactive 'Use interactive selection'
            cand -l 'List all matching directories'
            cand --list 'List all matching directories'
            cand -s 'Print score with results'
            cand --score 'Print score with results'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'querypath;remove'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
