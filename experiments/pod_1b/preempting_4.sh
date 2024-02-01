#!/bin/bash
#SBATCH -N 4
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q regular
#SBATCH -t 00:02:00
#SBATCH --reservation=pod_preempt_3
#SBATCH --job-name=patient_app

echo "4"
sleep 1m
