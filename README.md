# Algo-Phylo
Rust library for implementing algorithms for analyses in Bayesian phylogenetics.
We start with a Nexus and Newick parser to read in phylogenetic tree files.


## Parser
### NEXUS Format
Parses trees from a NEXUS file based on the TAXA block and TREES block (including TRANSLATE command), ignoring other blocks.

### Newick Strings
Parses Newick strings with (optional) branch lengths. Does not handle extra data in vertices yet (e.g. `[@...]`).

### Design
Uses a mapping from leaves in the tree structure to taxa names instead of saving labels multiple times (since in posterior samples we might have thousands of trees).


## Future Development
Well, more algos for analyses for Bayesian phylogenetics... Maybe next will be tractable tree distributions or cloudograms. But first the tree model needs to be extended with iterators and more.  
