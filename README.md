# Finance-Contracts

### Comandos
`cargo contract call --contract [Endereço] --message read_expense --args 0 --suri //Bob --skip-dry-run --output-json`

`cargo contract call --contract [Endereço] --message create_expense --args "\"Amazon Prime\"" "\"Assinatura mensal da Amazon Prime vídeo\"" 20 "\"08/02/2025\"" Lazer --suri //Bob --skip-confirm --execute`

`cargo contract call --contract [Endereço] --message update_expense --args 0 "\"Amz Prime\"" "\"Assinatura mensal da Amz Prime vídeo\"" 21 "\"10/02/2025\"" Lazer --suri //Bob --skip-confirm --execute`

`cargo contract call --contract [Endereço] --message delete_expense --args 1 --suri //Bob --skip-confirm --execute`