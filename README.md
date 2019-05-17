# pg_tool

A tool to automate and simplify postgres logical backups + extracting databases from them

## Implemented

- Running pg_dump and capturing output
- UI for progress
- Querying for database cluster size for an estimate. (not accurate, depends on cluster.)

## Required

- Either commit to pgpass and args, or use settings
- Make this tool generic for public use cases.

## Ideas

- Better way of logically dumping database rather than pg_dump