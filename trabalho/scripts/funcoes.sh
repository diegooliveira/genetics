

compila() {
    pushd ..
    cargo build --release
    popd
}

DATA_PATH=../latex/data
PDF_PATH=../latex/pdf

POPULACAO=100
GERACOES=500
POW=1


#
# Função de teste do percentual aplicado de mutação e seleção. Precisa que seja 
# informado a função objetivo e se deve ser aplicado elitismo
#
testa_mutacao() {

    FUNCAO=$1
    AGREGACAO=$2
    ELITISMO=$3
    
    SUFIX=""
    if [ "$ELITISMO" != "" ]
    then
        ELITISMO="--elitismo"
        SUFIX="_elitismo"
    fi
    
    echo "============================"
    echo "     Função: $FUNCAO"
    echo "    Mutação: $MUTACAO"
    echo " Cruzamento: $CRUZAMENTO"
    echo "   Gerações: $GERACOES"
    echo " Indivíduos: $POPULACAO"
    echo "   Elitismo: $ELITISMO"

    QTD_MUTACAO="0"

    DATA=$(realpath ${DATA_PATH}/${FUNCAO}_estudo_mutacao${SUFIX}.csv)
    PDF=$(realpath ${PDF_PATH}/${FUNCAO}_estudo_mutacao${SUFIX}.pdf)

    rm -rf $DATA $PDF

    echo "Vez;Mutacao;Cruzamento;Rastrigin" > $DATA

    for vez in `seq 1 20`
    do
        echo "Vez: $vez - Mut: $QTD_MUTACAO - Cruz: $CRUZAMENTO"
        for i in `seq 1 5`
        do
        	if [ "$i" == "5" ]
        	then
        		echo "  $i"
        	else
        		echo -n "  $i"
        	fi
        	
            RESULTADO=`../target/release/trabalho $ELITISMO \
                --cruzamento $CRUZAMENTO \
                --mutacao $QTD_MUTACAO  \
                --geracoes $GERACOES \
                --populacao $POPULACAO \
                --seletor torneio \
                --funcao ${FUNCAO} | tail -n 1`

            F=`echo $RESULTADO | cut -d';' -f 2 | awk '{printf("%.3f\n",$1)}' `
            echo "$vez;$QTD_MUTACAO;$QTD_CRUZAMENTO;$F" >> $DATA
        done
        QTD_MUTACAO=`expr "$vez" "*" "5"`
    done

    R --vanilla -q << EOM

valores<-read.csv("$DATA",sep=';')
medias=aggregate(valores\$Rastrigin^(1/${POW}), list(valores\$Mutacao), ${AGREGACAO})

pdf("$PDF", width=7, height=4)
plot(medias, type='l' , axes=FALSE, ann=FALSE, col='blue',
     panel.first = c(abline(h = 0, v = 0, col = "gray60"), grid(NA, 5, lwd = 2)))
axis(1, at = seq(0, 100, by = 5))
mtext("Mutação", side=1, line=3)
axis(2)
mtext("Menor Fitness", side=2, line=2)
dev.off()

EOM
    evince $PDF  &
}


gera_grafico(){

    DATA=$(realpath $1)
    PDF=$(realpath $2)
    
    R --vanilla -q  << EOM
        dados = read.csv("$DATA",sep=';')
        limits=range((dados\$Maior)^(1/${POW}), (dados\$Menor)^(1/${POW}), (dados\$Desvio)^(1/${POW}))
        pdf("$PDF", width=7, height=7)
        plot((dados\$Menor)^(1/${POW}), ylim=limits, col='red', bg='red', type='p', cex=0.4, pch=21,xlab="", ylab="", sub="",panel.first =
               c(abline(h = 0, v = 0, col = "gray60"), grid(NA, 5, lwd = 2)))
        lines((dados\$Media)^(1/${POW}), ylim=limits, col='yellow', bg='yellow', type='p', cex=0.4, pch=21)
        lines((dados\$Desvio)^(1/${POW}), ylim=limits, type='p', col='black', bg='black', cex=0.4, pch=21)
        lines((dados\$Maior)^(1/${POW}), ylim=limits, type='l', col='blue')
        mtext("Pop: $POPULACAO - Mut: $MUTACAO% - Cruz: $CRUZAMENTO%",side=1,line=2,at=c(50))
        
        dev.off()
EOM

}

# Função que gera os dados para o gráfico
avalia_distribuicao_adequacao() {

    SELETOR=$1
    FUNCAO=$2
    ELITISMO=$3
    SUFIX=""
    if [ "$ELITISMO" != "" ]
    then
        ELITISMO="--elitismo"
        SUFIX="_elitismo"
    fi
    
    CSV="dados_${SELETOR}_${FUNCAO}${SUFIX}"
    rm -rf  {$DATA_PATH,$PDF_PATH}/"${CSV}*"
    
    echo "============================"
    echo "     Função: $FUNCAO"
    echo "    Mutação: $MUTACAO"
    echo " Cruzamento: $CRUZAMENTO"
    echo "   Gerações: $GERACOES"
    echo " Indivíduos: $POPULACAO"

    ../target/release/trabalho $ELITISMO \
        --cruzamento $CRUZAMENTO \
        --mutacao $MUTACAO \
        --geracoes $GERACOES \
        --populacao $POPULACAO \
        --seletor ${SELETOR} \
        --funcao $FUNCAO > $DATA_PATH/${CSV}.csv

    gera_grafico $DATA_PATH/${CSV}.csv $PDF_PATH/${CSV}.pdf
    evince "$PDF_PATH/${CSV}.pdf" &
}

