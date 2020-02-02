/**
 * @module remote.js
 * @author Benedict R. Gaster
 * @copyright Benedict R. Gaster 2019
 *
 * FARM'19 version as part of SVG Interface Demo(s).
 * 
 * web-socket with MIDI remote controller for your presentation.
 *
 * @license: See LICENCE
 */
var start = new Date();

(function(window) {
    /**
     * Check that browser supports Web socket API, so far tested on
     *   Chrome and Firefox
     */
    if ("WebSocket" in window) {
	     console.log("Initializing ARemote...")

	      // open up remote server
        var ws = new WebSocket('ws://localhost:3000');

	      ws.onopen = function() {
            // Web Socket is connected, send data using send()
            // currently nothing to send
        }

        /**
         * Handle an incomming button press
         * @param  {[type]} evt data payload is midi CC messages,
         * currently 102 (left), 103 (right), 104 (up), and 105 (down)
         * mapping for slide movement
         */
        ws.onmessage = function (evt) {
            // cheap debouncing (hack alert...)
            var time = new Date() - start

            // only handle events 1sec apart, in truth this mostly works fine
            // but does mean you can't rush through the slides
            var midi = JSON.parse(evt.data);
            console.log(midi.midi);

            if (time > 1000) {
              switch(midi.midi[1]) {
                case 102:
                  Reveal.left()
                  break
                case 103:
                  Reveal.right()
                  break
                case 104:
                  Reveal.up()
                  break
                case 105:
                  Reveal.down()
                  break
              }

              // get a new time instance
              start = new Date();
            }
        }

        ws.onclose = function() {
            // websocket is closed.
        }
    }
})(window);
