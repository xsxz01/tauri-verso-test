import { Routes } from "@angular/router";

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'page',
        pathMatch: 'full'
    },
    {
        path: 'page',
        loadComponent: () => import('./pages/page.component').then((mod) => mod.PageComponent),
        children: [
            {
                path: '',
                redirectTo: 'home',
                pathMatch: 'full'
            },
            {
                path: 'home',
                loadComponent: () => import('./pages/home/home.component').then((mod) => mod.HomeComponent),
            }
        ]
    }

];
