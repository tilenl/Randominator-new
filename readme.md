# 🤖 Randominator V3
### Rust edition

## Version 3 news
This is a rewrite of the orginial Nim language codebase of Randominator V2.
By going with Rust, I was able to eliminate all bugs and retain the functionality of the original.
Flags parsing was redone and is now much more user friendly.

## Breaking changes with Version 2
 - some flags names have been changed
 - no more default dataset (you must always specify which file contains data)
 - JSON file format was replaced with TOML file format (for datasets)
 - double spacebars are not tolerated in output strings (they are replaced with one spacebar)
 - templates and data now reside in one file (because of templates and data missmatching previously)

## Installation
    cargo install --git https://github.com/tilenl/Randominator-new.git --branch main

If this does not work, you can still download the code from github, unzip it change directory to the folder (example: cd randominator-new), and execute this command inside it:

    cargo install --path .

## Example

### Dataset (*.toml file)
    ----- example.toml -----
    [templates]
    example = "<!greetings>, <!sentence>"
    example_eng = "<!greetings.eng>, <!question.eng>"

    [data]
        [data.greetings]
        eng = ["Hello", "Hi"]
        frn = ["Bonjour", "Salut"]
        slo = ["Pozdrav", "Zivijo"]
        ita = ["Buongiorno", "Ciao"]

        [data.question]
        eng = ["how are you?", "how is it going?"]
        slo = ["kako si?"]
        ita = ["come va?"]
    --End of Example.toml--
 - if a non final table (it has more subtables) is specified, randominator will randomly choose one path and continue along it, until it reaches parsable data (strings, arrays, ints, floats...), which he will use
 - if table contains subtables and data, data should always be in front of subtables. Otherwise that data will be grouped as a child of that subtable and not he original intended table:
 - you always need to specify: generate always (use '!') or generate sometimes (use '?')
 
This will work as intended

    [foo]                              
        foo_data = "Foo"            
        [foo.bar]                       
            bar_data = ["B", "Arr"]  
 
This will not work as intended (foo_data will be gruped with bar)

    [foo]                              
        [foo.bar]                       
            bar_data = ["B", "Arr"]  
        foo_data = "Foo"            

Some outputs when run as "randominator example.toml -n 3 -t example":
Hello, come va?
Hello, how is it going?
Buongiorno, kako si?

Some outputs when run as "randominator example.toml -n 3 -t example_eng":
Hello, how is it going?
Hi, how are you?
Hello, how are you?


To learn more out toml, [click this link](https://learnxinyminutes.com/docs/toml/)