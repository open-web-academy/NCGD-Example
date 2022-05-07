ID=dev-1651695167198-13781646549736
echo $ID

Inicializar contrato:

near call $ID init_contract '{"owner_id":"'$ID'"}' --accountId $ID

Obtener todas las puntuaciones:

    near view $ID obtener_puntuaciones

Obtener puntuacion de jugador:

    near call $ID obtener_puntuacion '{}' --accountId yairnava.testnet

Guardar puntuación

    near call $ID guardar_puntuacion '{"puntuacion": 30}' --accountId yairnava.testnet
