import matplotlib.pyplot as plt
import matplotlib.patches as patches
import numpy as np

def draw_architecture():
    fig, ax = plt.subplots(figsize=(10, 6))
    ax.axis('off')

    # Draw boxes
    boxes = {
        'Client / Web Interface': (0.1, 0.4, 0.2, 0.2),
        'Axum Web Server\n(api.rs)': (0.4, 0.4, 0.2, 0.2),
        'WarehouseState\n(Arc<RwLock>)\nGrid & Robots': (0.7, 0.6, 0.2, 0.2),
        'A* Pathfinding\n(pathfinding.rs)': (0.7, 0.2, 0.2, 0.2)
    }

    for name, (x, y, w, h) in boxes.items():
        rect = patches.Rectangle((x, y), w, h, linewidth=2, edgecolor='black', facecolor='#e0f7fa', zorder=2)
        ax.add_patch(rect)
        ax.text(x + w/2, y + h/2, name, ha='center', va='center', fontsize=12, fontweight='bold', zorder=3)

    # Draw arrows
    ax.annotate('', xy=(0.4, 0.5), xytext=(0.3, 0.5), arrowprops=dict(facecolor='black', width=2, headwidth=10))
    ax.text(0.35, 0.53, 'REST API', ha='center', fontsize=10)

    ax.annotate('', xy=(0.7, 0.7), xytext=(0.6, 0.5), arrowprops=dict(facecolor='black', width=2, headwidth=10))
    ax.text(0.6, 0.65, 'Read/Write State', ha='center', fontsize=10, rotation=30)

    ax.annotate('', xy=(0.7, 0.3), xytext=(0.6, 0.5), arrowprops=dict(facecolor='black', width=2, headwidth=10))
    ax.text(0.6, 0.35, 'Call A*', ha='center', fontsize=10, rotation=-30)

    ax.annotate('', xy=(0.8, 0.6), xytext=(0.8, 0.4), arrowprops=dict(facecolor='black', width=2, headwidth=10))
    ax.text(0.82, 0.5, 'Check Collisions', ha='left', fontsize=10)

    plt.title("System Architecture (Rust API)", fontsize=16, fontweight='bold')
    plt.tight_layout()
    plt.savefig('architecture_diagram.png', dpi=300)
    plt.close()

def draw_grid_collision():
    fig, ax = plt.subplots(figsize=(8, 8))
    
    # Create an empty 10x10 grid
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 10)
    ax.set_xticks(np.arange(0, 11, 1))
    ax.set_yticks(np.arange(0, 11, 1))
    ax.grid(True, linestyle='--', alpha=0.6)
    
    # Draw static obstacles
    obstacles = [(2, 2), (2, 3), (2, 4), (5, 5), (6, 5)]
    for ox, oy in obstacles:
        rect = patches.Rectangle((ox, oy), 1, 1, facecolor='gray')
        ax.add_patch(rect)
        
    # Draw Robot 1 (Reserved path)
    r1_path = [(1, 8), (2, 8), (3, 8), (4, 8), (4, 7)]
    for i in range(len(r1_path)-1):
        x1, y1 = r1_path[i]
        x2, y2 = r1_path[i+1]
        ax.plot([x1+0.5, x2+0.5], [y1+0.5, y2+0.5], 'r-', linewidth=3, zorder=2)
    ax.scatter(r1_path[0][0]+0.5, r1_path[0][1]+0.5, c='red', s=200, marker='s', label='Robot 1', zorder=3)
    ax.scatter(r1_path[-1][0]+0.5, r1_path[-1][1]+0.5, c='red', s=200, marker='X', label='R1 Goal', zorder=3)
    
    # Draw Robot 2 (Avoiding R1's path)
    r2_path = [(4, 9), (4, 10), (5, 10), (6, 10), (6, 9), (6, 8)]
    for i in range(len(r2_path)-1):
        x1, y1 = r2_path[i]
        x2, y2 = r2_path[i+1]
        ax.plot([x1+0.5, x2+0.5], [y1+0.5, y2+0.5], 'b-', linewidth=3, zorder=2)
    ax.scatter(r2_path[0][0]+0.5, r2_path[0][1]+0.5, c='blue', s=200, marker='s', label='Robot 2', zorder=3)
    ax.scatter(r2_path[-1][0]+0.5, r2_path[-1][1]+0.5, c='blue', s=200, marker='X', label='R2 Goal', zorder=3)
    
    # Legend
    ax.plot([], [], color='gray', marker='s', linestyle='None', markersize=10, label='Obstacle')
    ax.legend(loc='upper right', bbox_to_anchor=(1.25, 1))
    
    plt.title("Collision Avoidance on Grid", fontsize=16, fontweight='bold')
    plt.tight_layout()
    plt.savefig('collision_avoidance.png', dpi=300)
    plt.close()

if __name__ == '__main__':
    print("Generating diagrams...")
    draw_architecture()
    draw_grid_collision()
    print("Generated architecture_diagram.png and collision_avoidance.png!")
