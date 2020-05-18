# Introduction

This folder is intended to contain a collection of problem statements solved from the little book of semaphores using Rust. The blocks are split chapter wise in the original book.

Link : http://greenteapress.com/semaphores/LittleBookOfSemaphores.pdf

## ch1

### s_1_3_sync_threads_with_crossbeam_messages
### s_1_3_sync_futures_with_crossbeam_messages

Serialization: Event A must happen before Event B.

### s_1_5_1_concurrent_updates_futures_to_arc_mutex

Concurrent read / write to arc mutex ( shared variable )

## ch3 

### s_3_3_1_crossbeam_rendezvous_signal_pattern

Generalize the signal pattern so that it works both ways. Thread A has to wait for Thread B and vice versa.
