TODO ADIOS, DESDE 0

09 - septiembre - 2026
	Chau proyecto, todo desde 0 🤑🤑

	Plan:

	 1. Servidor<-cliente (Cliente manda cadena de texto por websocket, servidor la imprime :D)
			Websockets, conexiones y almacenamiento de usuarios
	 2. Servidor<-muchos clientes (Clientes mandan cadenas de texto por websocket, servidor las imprime :D) [ASINCRONO]
			Programacion asincrona, diccionario de usuarios con username y status
			Ademas, el servidor y el cliente deben terminar de forma limpia
	 3. Servidor y Cliente procesador (Cliente manda cadena de texto por websocket, servidor la desenvuelve y ejecuta funcion adecuada) [JSON CADENAS]
			Hacer un desenvolvedor de json y envolvedor de json. Mandar json cadenas y descomponerlas de cada lado
	 4. Cliente<-servidor (Servidor manda cadena de texto por websocket, cliente la imprime :D)
			Empezar con un controlador basico, solo imprime una cadena dada
	 5. Cliente<->cliente (Cliente manda cadena de texto por websocket, servidor la procesa y se la manda a otro cliente) [TOMAR Y MANDAR MENSAJES]
			Servidor puede recibir un mensaje de un usuario y mandarselo a otro procesado de forma adecuada. Varios mandan mensajes al mismo tiempo y servidor sabe que mensaje va a donde
	 6. Cliente->El resto de clientes (Cliente manda cadena de texto por websocket, el resto de clientes las reciben) [TOMAR UN SOLO MENSAJE Y REPETIRLO EN GLOBAL]
			Hacer un for-each de diccionario de usuarios para poder dispersar texto
	 7. Salas (Crear salas, filtrar mensajes para solo mandarse a usuarios de una sala) [FILTRAR A QUIEN MANDAR MENSAJES]
			Filtros de usuarios, para que un mensaje solo alcance X usuarios
	 8. Protocolo/comandos (Implementar protocolo en servidor y cliente con lo que ya llevamos)
			Empezar a implementar bien el protocolo (?)
	 9. Cliente controlador (Terminar controlador cliente) [CONFORME AVANCE EL PROTOCOLO]
			Terminar el controlador basico que empezo desde que el cliente imprime texto
	10. Cliente terminal (imprimir notificaciones en terminal)
			Con el controlador bien hecho, ya con input y todo, hacer una clase que tome lo que le diga el controlador y poder formatearlo para imprimir en terminal
	11. Volverlo Online
			Probar servidor en linea real
	========== GOAL ==========
	12. Cliente GUI (Interfaz grafica cliente)
			Reemplazar cliente terminal con GUI
	13. Servidor UI (Logging de servidor a terminal)
			Implementar servidor controlador y imprimir todo el trafico que sucede dentro del servidor
