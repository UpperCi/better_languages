#include <stdio.h>
#include "vertalingen_nl.h"

// sorts input string alphabetically

hgt bubbel(kar *reeks) {
	hgt gesorteerd = 1;
	kar vorige = 0;

	zolang (reeks[0] != '\0') {
		kar k = reeks[0];

		indien (k < vorige) {
			reeks[0] = vorige;
			reeks[-1] = k;
			gesorteerd = 0;
		} anders {
			vorige = k;
		}

		reeks++;
	}

	retourneer gesorteerd;
}

hgt hoofd(hgt aarg, kar **argw) {
	kar *reeks = argw[1];
	doe {} zolang (!bubbel(reeks));
	zetr(reeks);
}
