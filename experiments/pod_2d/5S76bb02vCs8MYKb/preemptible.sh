#!/bin/bash

#SBATCH -N 2
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q sow
#SBATCH -t 1:00:00
#SBATCH --signal=R:INT@60
#SBATCH --job-name=patient_app
#SBATCH --nodelist=nid[200000-200004,200006-200030,200032-200058,200060-200066]

srun -n 1 ~/slurm-preemption-experiments/jobs/patient_app
