set -e

source ./funcoes.sh
source ./rastrigin.sh

compila


avalia_distribuicao_adequacao "torneio" "rastrigin_binario"  "elitismo"
avalia_distribuicao_adequacao "torneio" "rastrigin_binario"  ""
avalia_distribuicao_adequacao "torneio" "rastrigin_arranjo"  "elitismo"
avalia_distribuicao_adequacao "torneio" "rastrigin_arranjo"  ""
avalia_distribuicao_adequacao "roleta" "rastrigin_binario"  "elitismo"
avalia_distribuicao_adequacao "roleta" "rastrigin_binario"  ""
avalia_distribuicao_adequacao "roleta" "rastrigin_arranjo"  "elitismo"
avalia_distribuicao_adequacao "roleta" "rastrigin_arranjo"  ""
