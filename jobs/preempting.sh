#!/bin/bash
#SBATCH -N 4
#SBATCH -A nstaff_g
#SBATCH -C gpu
#SBATCH -q regular
#SBATCH -t 00:02:00
#SBATCH --reservation=preemption_test_perlmutter
#SBATCH --job-name=patient_app

sleep 1m
