# npm_package_cheker

This is a check to see if there are any
problem dependencies included in a package
without having to have it installed. 


## Get Package Data

get-package-data.rs

Pulls down the package and its dependencies
into a JSON file for processing.


## Prepping Bad Packages JSON

prep-bad-packages.rs

Only need to run this if the source data changes
from https://github.com/devbyray/check-vulnerable-npm-packages. 
You'll need to update the rust source code
since the data is baked into the binary. 

When you run the process it outputs the data
file into the current directory. It needs
to be moved to `src/data/bad-packages.json`
then the `check-for-problems.rs` file needs
to be recompiled to include it. 



## Notes

Resources

- https://github.com/npm/registry/blob/main/docs/REGISTRY-API.md



