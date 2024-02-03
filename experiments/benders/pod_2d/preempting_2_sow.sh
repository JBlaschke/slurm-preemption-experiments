#!/bin/bash
#SBATCH -N 2
#SBATCH -A nstaff
#SBATCH -C cpu
#SBATCH -q sow
#SBATCH -t 00:02:00
#SBATCH --nodelist=nid[200000-200001,200003-200004,200006-200030,200033,200037,200039-200040,200043-200044,200048,200050-200051,200054,200059,200063,200070,200073,200077,200087,200089,200113-200114,200123,200135,200141,200143-200144,200161,200168,200175,200181,200183,200187,200191,200196-200197,200202,200209]
#SBATCH --job-name=patient_app

echo "sow 2"
sleep 1m
