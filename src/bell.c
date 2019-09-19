/*	$OpenBSD: bell.c,v 1.5 2019/07/17 18:18:37 lum Exp $	*/

/*
 * This file is in the public domain.
 *
 * Author: Mark Lumsden <mark@showcomplex.com>
 *
 */

/*
 * Control how mg communicates with the user.
 */

#include <sys/queue.h>
#include <signal.h>
#include <stdio.h>
#include <unistd.h>

#include "def.h"
#include "macro.h"

void
bellinit(void)
{
	doaudiblebell = 1;
	dovisiblebell = 0;
}

int
dobeep_msgs(const char *msg, const char *s)
{
	ewprintf("%s %s", msg, s);
	dobeep();
	return (FALSE);
}

int
dobeep_msg(const char *msg)
{
	ewprintf("%s", msg);
	dobeep();
	return (FALSE);
}

void
dobeep(void)
{
	if (doaudiblebell) {
		ttbeep();
	}
	if (dovisiblebell) {
		sgarbf = TRUE;
		update(CNONE);
		if (inmacro)	/* avoid delaying macro execution. */
			return;
		usleep(50000);
	}
}

/* ARGSUSED */
int
toggleaudiblebell(int f, int n)
{
	if (f & FFARG)
		doaudiblebell = n > 0;
	else
		doaudiblebell = !doaudiblebell;

	return (TRUE);
}

/* ARGSUSED */
int
togglevisiblebell(int f, int n)
{
	if (f & FFARG)
		dovisiblebell = n > 0;
	else
		dovisiblebell = !dovisiblebell;

	return (TRUE);
}
