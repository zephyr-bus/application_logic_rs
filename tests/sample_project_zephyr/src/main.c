/*
 * Copyright (c) 2012-2014 Wind River Systems, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr.h>
#include <sys/printk.h>

#include "application_logic_rs.h"

void rs_msleep(int32_t ms) {
	k_msleep(ms);
}

void main(void)
{
  printk("Hello World from C!\n");
  int32_t t[] = {2, 2, 2, 2, 2};
  printk("smean_rs = %d\n", smean_rs(1, t, sizeof(t) / sizeof(t[0])));
  k_msleep(3000);
  application_logic();
}
