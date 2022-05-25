ID=dev-1652728160134-47102512188392
echo $ID

Inicializar contrato:

near call $ID init_contract '{"owner_id":"'$ID'"}' --accountId $ID

Obtener todas las puntuaciones:

    near view $ID obtener_puntuaciones

Obtener puntuacion de jugador:

    near call $ID obtener_puntuacion '{"owner_id":"'yairnava.testnet'"}' --accountId yairnava.testnet

Guardar puntuación

    near call $ID guardar_puntuacion '{"puntuacion": 40}' --accountId yairnava.testnet
