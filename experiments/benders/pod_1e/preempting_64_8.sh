#!/bin/bash
#SBATCH -N 64
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q regular
#SBATCH -t 00:08:00
#SBATCH --reservation=pod_preempt_5
#SBATCH --job-name=patient_app

echo "8 64"
sleep 4m
