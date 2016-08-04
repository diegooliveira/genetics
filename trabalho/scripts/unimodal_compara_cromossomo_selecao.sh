
source ./funcoes.sh
source ./unimodal.sh

compila

#avalia_distribuicao_adequacao "torneio" "unimodal_arranjo_um"  ""
#avalia_distribuicao_adequacao "roleta" "unimodal_arranjo_um"  ""
#avalia_distribuicao_adequacao "torneio" "unimodal_arranjo_um"  "elitismo"
#avalia_distribuicao_adequacao "roleta" "unimodal_arranjo_um"  "elitismo"

avalia_distribuicao_adequacao "roleta" "unimodal_arranjo_dois"  "elitismo"
avalia_distribuicao_adequacao "roleta" "unimodal_arranjo_dois"  ""
avalia_distribuicao_adequacao "torneio" "unimodal_arranjo_dois"  "elitismo"
avalia_distribuicao_adequacao "torneio" "unimodal_arranjo_dois"  ""