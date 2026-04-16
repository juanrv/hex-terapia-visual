# Requisitos del Sistema

Este documento detalla los requisitos funcionales, no funcionales y las consideraciones de diseño para el desarrollo de la aplicación VisualTherapy.

## 1. Objetivo

Crear una herramienta de terapia visual que funcione como una capa de software transparente sobre el sistema operativo, permitiendo al usuario realizar sus tareas cotidianas mientras se entrena la visión.

## 2. Requisitos Funcionales (RF)

### Módulo de Gestión de Terapias

- **RF-01**: La aplicación debe mostrar un menú principal al inicio para seleccionar el tipo de terapia.
  - **Versión actual (MVP)**: Terapia de División de Color con múltiples layouts.

  - **Extensibilidad futura**: La arquitectura debe permitir añadir nuevos tipos de terapia (ej. filtros, patrones, ejercicios dinámicos) sin necesidad de reescribir el núcleo de la aplicación.

### Módulo de Superposición (Core)

- **RF-02**: Generar una ventana de capa superior (overlay) que cubra toda la pantalla, independiente de la resolución.

- **RF-03**: Dividir el área de la pantalla en dos zonas configurables.

- **RF-04**: Permitir el "clic a través" (click-through) en las zonas de terapia para que el usuario pueda interactuar con las ventanas subyacentes.

### Módulo de Configuración Avanzada

- **RF-05**: Permitir la selección de colores personalizados para cada zona mediante herramientas avanzadas (RGB, selectores de color).

- **RF-06**: Control de opacidad/transparencia independiente para cada zona de color. Esto permite equilibrar diferencias de luminosidad entre ojos.

### Interfaz y Control

- **RF-07**: La aplicación proporcionará controles mediante bandeja del sistema como método principal. Los botones flotantes se implementarán solo si la complejidad técnica no compromete la estabilidad general.

- **RF-08**: Funcionalidad de minimizado a la bandeja del sistema (System Tray) para liberar espacio en la barra de tareas.

### Persistencia de Datos

- **RF-09**:La aplicación debe guardar la última configuración utilizada (colores, opacidades, tipo de terapia, layout seleccionado) <u>al cerrarse la aplicación</u>. No es necesario guardar automáticamente cada cambio individual.

### Control de Estado

- **RF-10**:La aplicación debe conocer en todo momento si la terapia está activa o detenida, y reflejar este estado en:
  - El texto/menú de la bandeja del sistema
  - El botón flotante de inicio/detención
  - Los atajos de teclado (si están implementados)

  No debe ser posible "iniciar" dos veces la misma terapia ni "detener" cuando ya está detenida.

### Módulo de Layouts

- **RF-11**: La aplicación debe soportar diferentes disposiciones de las zonas de color:
  - División vertical (izquierda/derecha)
  - División horizontal (arriba/abajo)
  - Otras disposiciones que se implementen en el futuro

- **RF-12**: El usuario debe poder seleccionar el layout deseado desde la interfaz de control.

- **RF-13**: Cada zona de color en cualquier layout debe mantener sus propios ajustes de color y opacidad independientes.

- **RF-14**: La aplicación debe permitir modificar los parámetros de la terapia (colores, opacidades, layout) mientras está activa, aplicando los cambios en la superposición de forma inmediata.

## 3. Requisitos No Funcionales (RNF)

### Plataforma y Compatibilidad

- **RNF-01**: Prioridad de desarrollo para Windows (10/11).

- **RNF-02**: Compatibilidad futura con Linux. El código debe evitar dependencias exclusivas del sistema operativo en la lógica de negocio (uso de adaptadores/abstracciones).

### Rendimiento

- **RNF-03**: Bajo consumo de recursos. La aplicación debe ejecutarse con un impacto mínimo en la CPU y la RAM para no ralentizar el uso normal del ordenador.

- **RNF-04**: Inicio rápido y respuesta inmediata a los controles (latencia baja).

### Arquitectura

- **RNF-05** - Separación de responsabilidades: La aplicación debe organizarse en tres capas con reglas claras de dependencia:
  - Dominio: Contiene la lógica específica de las terapias visuales. Esta capa NO debe conocer cómo se muestran las cosas en pantalla, cómo se guarda la configuración, ni qué sistema operativo se está usando.

  - Controladores de entrada: Manejan lo que el usuario hace (clic en botones, teclas de acceso rápido). Traducen las acciones del usuario en órdenes para las reglas del negocio.

  - Adaptadores de salida: Manejan cómo la aplicación interactúa con el exterior (mostrar ventanas, guardar archivos, reproducir sonidos). Las reglas del negocio les piden que hagan algo, pero no saben CÓMO lo hacen.

- **RNF-06**: Uso de archivo de configuración estándar (JSON o TOML) para evitar dependencias de bases de datos complejas.

### Distribución y Portabilidad

- **RNF-07**: La aplicación debe ser portable. No debe requerir un proceso de instalación complejo por parte del usuario (sin asistentes de instalación "Next-Next-Finish"). El objetivo es que el usuario pueda ejecutar la aplicación directamente (ej. desde una memoria USB) sin derechos de administrador.

  > **Nota:** En Windows, esto se traduce en un ejecutable .exe autónomo. En Linux, un binario portable o AppImage.

### Comportamiento Ante Fallos

- **RNF-08**: Tolerancia a fallos
  - Si la aplicación no puede crear la capa de terapia (ej. por falta de permisos), debe mostrar un mensaje claro al usuario y funcionar en modo "solo configuración".
  - Si el archivo de configuración está dañado, debe iniciar con valores por defecto y notificar al usuario.
  - Si la aplicación se cierra inesperadamente, la última configuración activa debe poder recuperarse.

### Pruebas

- **RNF-09**: La aplicación debe diseñarse de forma que se pueda probar el funcionamiento de las terapias sin necesidad de una pantalla real. Esto implica que la lógica de las terapias no debe depender directamente de cómo se dibujan los colores en la pantalla.

## 4. Accesibilidad y UX

Dado el público objetivo, la accesibilidad es crítica.

- **Escalado de UI**: La interfaz debe soportar escalado nativo del sistema (DPI Awareness) y ofrecer un control manual para ajustar el tamaño de los elementos visuales.
- **Navegación por Teclado**: Implementar atajos globales (Hotkeys) para funciones críticas (ej. Detener terapia inmediatamente) sin necesidad de usar el ratón.
- **Alto Contraste**: Los elementos de control (botones flotantes, menús) deben tener bordes definidos o temas de alto contraste para ser visibles sobre cualquier fondo.
- **Feedback Auditivo**: Opcionalmente, sonidos de confirmación al iniciar/detener sesiones.

## 5. Consideraciones Técnicas Clave

El problema del "Click-through" vs "Botones Flotantes"

Para lograr que la terapia sea transparente al ratón pero los botones de control sean clicables, se requiere una arquitectura de dos ventanas separadas:

- Ventana de Terapia: Pantalla completa, atributo "Click-through" activado (el ratón la ignora).
- Ventana de Control: Ventana pequeña flotante, atributo "Top-most" (siempre visible), atributo "Click-through" desactivado (interactiva).

Esta separación garantiza que el usuario pueda hacer clic en los botones de control sin perder la capacidad de usar las aplicaciones que están debajo de la capa de terapia.
