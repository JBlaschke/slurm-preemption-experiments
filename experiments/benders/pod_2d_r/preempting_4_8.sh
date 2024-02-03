#!/bin/bash
#SBATCH -N 4
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q regular
#SBATCH -t 00:08:00
#SBATCH --reservation=pod_preempt_5
#SBATCH --job-name=patient_app

echo "8 4"
sleep 4m
