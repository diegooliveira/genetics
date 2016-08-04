
source ./funcoes.sh
source ./multimodal.sh

compila


testa_mutacao "multimodal_arranjo" "max" "elitismo"
testa_mutacao "multimodal_arranjo" "max" ""
testa_mutacao "multimodal_arranjo_binario" "max" "elitismo"
testa_mutacao "multimodal_arranjo_binario" "max" ""
 

