#!/bin/bash
#SBATCH -N 16
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q regular
#SBATCH -t 00:02:00
#SBATCH --reservation=pod_preempt_5
#SBATCH --job-name=patient_app

echo "16"
sleep 1m
