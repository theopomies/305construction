# 305 Construction

The goal of this project is to create a project management software that helps organize construction, based on the MPM method. Starting from a file that describes all of the project tasks, we will display:
- Total duration of construction,
- The earliest and latest start dates for each task,
- A Gantt chart that specifies intervals of fluctuation.

## How to build
```sh
$ make re
```

## Examples
```sh
$ ./305construction -h
USAGE
    ./305construction file
DESCRIPTION
    file    file describing the tasks
```
```sh
$ cat house.csv
Car;carpenter;4;Fou
Hea;heat;3;Ele;Mas
Cov;cover;2;Car
Ele;electricity;1;Cov;Mas
Fin;finishing touches;9;Cov;Ele;Mas;Plu
Fou;foundations;8;Lan
Mas;masonry;4;Lan;Fou
Plu;plumbing;1;Ele;Mas
Lan;landscaping;3
```
```sh
$ ./305construction house.csv
Total duration of construction: 28 weeks

Lan must begin at t=0
Fou must begin at t=3
Car must begin at t=11
Mas must begin between t=11 and t=13
Cov must begin at t=15
Ele must begin at t=17
Plu must begin at t=18
Hea must begin between t=18 and t=25
Fin must begin at t=19

Lan     (0)     ===
Fou     (0)        ========
Car     (0)                ====
Mas     (2)                ====
Cov     (0)                    ==
Ele     (0)                      =
Plu     (0)                       =
Hea     (7)                       ===
Fin     (0)                        =========

```