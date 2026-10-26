// Core agent: page-wide behaviours — the one drawer and stepper, the offline banner, the
// navigation progress/failure UI (with the router), prefs links keeping the live address,
// pull-to-refresh.

import { register } from "../core/mount";
import { mountDrawer } from "../core/drawer";
import { mountStepper } from "../core/stepper";

register("drawer", mountDrawer);
register("stepper", mountStepper);
