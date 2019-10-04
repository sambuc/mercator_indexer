# Mercator Indexer

Small tool to generate indexes for the Mercator Spatial Index service.

## Mercator: Spatial Index

**Mercator** is a spatial *volumetric* index for the [Human Brain Project](http://www.humanbrainproject.eu). It is a component of the [Knowledge Graph](http://www.humanbrainproject.eu/en/explore-the-brain/search/) service, which  provides the spatial anchoring for the metadata registered as well as processes the volumetric queries.

It is build on top of the Iron Sea database toolkit.

## Iron Sea: Database Toolkit

**Iron Sea** provides a set of database engine bricks, which can be combined and applied on arbitrary data structures.

Unlike a traditional database, it does not assume a specific physical structure for the tables nor the records, but relies on the developper to provide a set of extractor functions which are used by the specific indices provided.

This enables the index implementations to be agnostic from the underlying data structure, and re-used.

## Requirements

### Software

 * Rust: https://www.rust-lang.org

## Quick start

Checkout the dependencies in the parent folder:
 * mercator_db
 * ironsea_index
 * ironsea_store
 * ironsea_table
 * ironsea_index_hashmap
 * ironsea_index_sfc_dbc
 * ironsea_table_vector

For 3 datasets, `setA`, `setB`, `setC`, the following files are expected to be in the current folder:
 * setA:
   - setA.objects.json
   - setA.spaces.json
 * setB:
   - setB.objects.json
   - setB.spaces.json
 * setC:
   - setC.objects.json
   - setC.spaces.json

Run (and build if necessary) the indexer:

```sh
cargo run --release -- setA setB setC
```

This will produce the following files:
 * setA
   - setA.objects.bin
   - setA.spaces.bin
   - setA.index
 * setB
   - setB.objects.bin
   - setB.spaces.bin
   - setB.index
 * setC
   - setC.objects.bin
   - setC.spaces.bin
   - setC.index
   
By default, each dataset will have a version set to the empty string, if you want to specify the dataset version you can like this:

```sh
cargo run --release -- setA:v0.1 setB setC:MyAwesomeVersion
```

With the above, `setA` will have its version set to `v0.1`, `setB` to the empty string and `setC` to `MyAwesomeVersion`. 

## Acknowledgements

This open source software code was developed in part or in whole in the
Human Brain Project, funded from the European Union’s Horizon 2020
Framework Programme for Research and Innovation under the Specific Grant
Agreement No. 785907 (Human Brain Project SGA2).
