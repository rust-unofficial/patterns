# Command

## Description

Suppose we have a sequence of actions or transactions
encapsulated as objects.
We want these actions or commands to be
executed or invoked in some order later at different time.
These commands may also be triggered as a result
of some event. For example when a user pushes a button
or on arrival of a data packet.
In addition, these commands might be be undoable.
For example, operations of the editor.
We also may want to store logs of executed commands
so that we could reapply the changes later if system crashes.

## Motivation

Define two database operations
`create table` and `add field`.
Each of these operations is a command
which knows how undo the command, e.g.,
`drop table` and `remove field`.
When a user invokes a database migration
operation then each command is executed in the defined order,
and when the user invokes the rollback operation
then each undo operation is invoked in reverse order.
