#!/bin/bash

#SBATCH -N 4
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q regular
#SBATCH -t 1:00:00
#SBATCH --signal=R:INT@60
#SBATCH --job-name=patient_app
#SBATCH --reservation=pod_preempt_4

srun -n 1 ~/slurm-preemption-experiments/jobs/patient_app
