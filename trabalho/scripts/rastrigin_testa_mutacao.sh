

source ./funcoes.sh
source ./rastrigin.sh

compila
 

testa_mutacao "rastrigin_binario" "min" "elitismo" 
testa_mutacao "rastrigin_binario" "min" "" 

