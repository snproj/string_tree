# string_tree
A parser for a verb-subject-object (VSO) brand of English. It takes in VSO English and outputs it indented appropriately. It's also supposed to create an object representation of events, people, time, and their relationships, though this is unfinished as of now. It's the next step after `steno` (in a different repo).

## Usage
In its current state the input is hardcoded, so it's not very useful unless you modify the code itself.

## Purpose
Being interested in human linguistics, and thinking of conlangs (constructed/fictional languages) for my book, I came up with an interesting concept for a language that's completely VSO in structure.

Further contemplation made me realize that if taken to the extreme, it was more like a language for computers than people. Also, parsing it would only require a stack machine; no punctuation or anything else was strictly required since the structure of the text was wholly and unambiguously contained within the strict grammar rules.

What would this be useful for? It was a solution looking for a problem, and I figured the one thing it might be sueful for was as a compressed method of storing human English in a way that's a bit more computer friendly.

As part of the object creation procedure, I also attempted multithreading a search down a binary tree. I decided to use Rust for this specifically to experience its supposed memory safety (and fighting the borrow checker :s) and to get familiar with the concept of mutexes and locks in general.