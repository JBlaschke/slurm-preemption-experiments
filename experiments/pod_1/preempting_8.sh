#!/bin/bash
#SBATCH -N 8
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q regular
#SBATCH -t 00:02:00
#SBATCH --reservation=pod_preempt_2
#SBATCH --job-name=patient_app

echo "8"
sleep 1m
