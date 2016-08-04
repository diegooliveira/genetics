
source ./funcoes.sh
source ./multimodal.sh

compila

avalia_distribuicao_adequacao "torneio" "multimodal_arranjo_binario"  ""
avalia_distribuicao_adequacao "torneio" "multimodal_arranjo_binario"  "elitismo"
avalia_distribuicao_adequacao "torneio" "multimodal_arranjo"  ""
avalia_distribuicao_adequacao "torneio" "multimodal_arranjo"  "elitismo"
