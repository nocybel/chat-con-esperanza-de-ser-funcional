Diario coso de avances de proyecto :p
(Un tipo cuaderno de notas con ideas. Posible guia para hacer el reporte final?)

28 - agosto - 2026

	Yayyy primer proyecto que miedo.
	Quiero tener este tipo "diario" de cosas que hago y tal para luego redactar algo legible como reporte. Esto va a servir más para aclarar mis propias ideas y llevar registro de cosas que hago/pienso (además del log de git).

	Hoy empieza el proyecto porque ya mandaron las especificaciones del protocolo de comunicación :D
	
	Primero hay que empezar haciendo el análisis del problema según el modelo de la cascada que dijeron que tenemos que usar.
	
	Entonces, Análisis:
		El punto de los dos programas (servidor y cliente) es basicamente mandar strings entre si por medio de internet y luego hacer cositas con esos Strings (wao, bien explicado)
		El cliente envía mensajes y espera (o no) una respuesta. Además, nunca le responde al servidor (muy triste porque si hubiera un ping podría autodesconectar usuarios si alguien pierde conexión por 10 pings seguidos o algo así). También tiene que ser bonito y presentar información de una buena manera al usuario pero eso lo precisaré después. A su nivel más básico, solo envía texto y procesa texto recibido para mostrarselo bien al usuario. Como extra, me gustaría que tenga interfaz gráfica, y eso implica tener pestañas para cada sala existente, una lista de usuarios que se actualiza cada tanto (y manualmente si quieres saber el instante en el que alguien se conecta o desconecta), un menu para mandar mensajes diferentes a texto, etc. Como digo, lo preciso mejor después
		
		Para el servidor, este suele responder a mensajes de clientes y envía los mensajes de un cliente al resto para que todos se enteren del chisme. También tiene que llevar cuenta de los usuarios (y cosas relacionados a ellos) y de las salas existentes.
		
		Podría hacer algo de trampa y crear comandos extra (mensajes en el sentido de Strings para comunicación cliente-servidor) que solo sirven con mi cliente para que algunas cosas sean más elegante, pero creo que no debo y además tomaría más tiempo para algo que no me piden.
		También estaría bien que el servidor tuviera un cliente integrado para que el dueño del server pueda hablar con los clientes (tipo mandar texto desde un usuario "Server" o algo asi)
		Quizás este cliente integrado debería tener la habilidad de desconectar usuarios, o de ver todos los mensajes existentes. Sería conveniente para poder moderar, pero igual es trabajo extra que seguramente rompe las reglas (pues depende de cómo se diseñe tendría que extender el protocolo de comunicacion para que sirva) y que a fin de cuentas no va a ser útil excepto para espíar por 2 minutos a tus amigos que están probando tu proyecto antes de regresar a discord de toda la vida.
		
	Por ahora se me ocurre eso.
	Tengo pésima ortografía (me salté chingo de acentos) y uso un lenguaje no muy apropiado pero eeeh igual este no es el reporte, son notas mias para mi mismo xd.
	
	Pues creo que he trabajado duro y arduamente el día de hoy y me merezco un descanso (jugar tf2). Debería avanzar al menos unas cositas perooo aun no se casi nada de Rust y tengo varias dudas que sería bueno aclarar antes de empezar en serio el proyecto.
	
	Dudas para el lunes que tenga clase con canek:
		- Reporte, cómo?
			Latex en archivo .tex o en .pdf o los 2?
			Tiene que ir al final o avanzar junto al proyecto?
			Debe ser personal como un diario o más "profesional" de que son especificaciones y muy objetivo?
		- Es posible _extender_ el protocolo para agregar cositas unicas a mi cliente?
		- Es buena idea tener un cliente integrado en el servidor?
	
	yei primer dia de proyecto, que emocion :3
